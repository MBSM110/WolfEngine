/*
    Project: Wolf Engine. Copyright © 2014-2023 Pooya Eimandar
    https://github.com/WolfEngine/WolfEngine
*/

#pragma once

#ifdef __SANITIZE_ADDRESS__
#ifndef _DISABLE_VECTOR_ANNOTATION
#define _DISABLE_VECTOR_ANNOTATION
#endif
#endif

#ifdef _MSC_VER
#include <codeanalysis\warnings.h>
#endif

#ifdef WIN32
#include <Windows.h>
#endif

#ifndef EMSCRIPTEN
#include <filesystem>
#endif

#include <chrono>
#include <exception>
#include <memory>
#include <optional>
#include <sstream>
#include <string>

#include <boost/leaf.hpp>

#ifdef _MSC_VER
#include <format>
#else
#include <fmt/chrono.h>
#include <fmt/format.h>
#endif

#include <gsl/gsl>

constexpr auto W_MAX_PATH = 260;
constexpr auto W_MAX_BUFFER_SIZE = 1024;

#define DEFER auto _ = std::shared_ptr<void>(nullptr, [&](...)

struct w_buffer {
  w_buffer() noexcept = default;

  explicit w_buffer(const std::string_view p_str) { from_string(p_str); }

  w_buffer(std::array<char, W_MAX_BUFFER_SIZE> &&p_array, const size_t p_used_bytes) noexcept {
    this->buf = std::move(p_array);
    this->used_bytes = p_used_bytes;
  }

  void from_string(const std::string_view p_str) {
    const auto _size = p_str.size();
    this->used_bytes = _size > 1024 ? 1024 : _size;
    std::copy(p_str.cbegin(), p_str.cbegin() + this->used_bytes, buf.begin());
  }

  std::string to_string() {
    if (this->buf.size() && this->used_bytes) {
      return std::string(this->buf.data(), this->used_bytes);
    }
    return std::string();
  }

  std::array<char, W_MAX_BUFFER_SIZE> buf = {0};
  size_t used_bytes = 0;
};

// #ifdef __clang__
// #define W_ALIGNMENT_16 __attribute__((packed)) __attribute__((aligned(16)))
// #define W_ALIGNMENT_32 __attribute__((packed)) __attribute__((aligned(32)))
// #define W_ALIGNMENT_64 __attribute__((packed)) __attribute__((aligned(64)))
// #define W_ALIGNMENT_128 __attribute__((packed)) __attribute__((aligned(128)))
// #else
// #define W_ALIGNMENT_16
// #define W_ALIGNMENT_32
// #define W_ALIGNMENT_64
// #define W_ALIGNMENT_128
// #endif

#ifdef _MSC_VER

#define W_API __declspec(dllexport)
#define ASM __asm

#else

#define W_API
#define ASM __asm__

// define dummy SAL
#define _In_
#define _In_z_
#define _Out_
#define _Inout_
#define _Inout_z_
#define _In_opt_
#define _In_opt_z_
#define _Inout_opt_

#endif

#include <wolf/system/w_trace.hpp>
#include <wolf/system/invocable.h>

namespace wolf {
  template <class T>
  using w_function = ofats::any_invocable<T>;
  using w_binary = std::vector<std::byte>;

/**
 * returns wolf version
 * @return string format with the following style
 * "<major>.<minor>.<patch>.<debug>"
 */
W_API std::string w_init();

/**
 * make a string via format
 * @param p_fmt, the fmt
 * @param p_args, the args
 * @return a string
 */
#ifdef _MSC_VER
template <class... Args>
W_API std::string format(_In_ const std::string_view p_fmt, _In_ Args &&...p_args) {
  return std::vformat(p_fmt, std::make_format_args(p_args...));
}
#else
template <class... Args>
W_API std::string format(_In_ const fmt::v9::format_string<Args...> p_fmt, _In_ Args &&...p_args) {
  return fmt::v9::vformat(p_fmt, fmt::v9::make_format_args(p_args...));
}
#endif  // _MSC_VER
}  // namespace wolf
