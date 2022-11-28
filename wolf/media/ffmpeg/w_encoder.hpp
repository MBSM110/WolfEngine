/*
    Project: Wolf Engine. Copyright © 2014-2022 Pooya Eimandar
    https://github.com/WolfEngine/WolfEngine
*/

#ifdef WOLF_MEDIA_FFMPEG

#pragma once

#include "w_av_frame.hpp"
#include "w_av_packet.hpp"
#include "w_ffmpeg_ctx.hpp"
#include <variant>

namespace wolf::media::ffmpeg {

struct w_encoder {
  w_ffmpeg_ctx ctx = {};

  // constructor
  W_API w_encoder() = default;
  // destructor
  W_API virtual ~w_encoder() noexcept = default;

  // move constructor.
  W_API w_encoder(w_encoder &&p_other) noexcept = default;
  // move assignment operator.
  W_API w_encoder &operator=(w_encoder &&p_other) noexcept = default;

  W_API boost::leaf::result<int> start(_In_ const w_av_frame &p_frame,
                                        _Inout_ w_av_packet &p_packet) noexcept;
private:
  // copy constructor
  w_encoder(const w_encoder &) = delete;
  // copy operator
  w_encoder &operator=(const w_encoder &) = delete;
};
} // namespace wolf::media::ffmpeg

#endif // WOLF_MEDIA_FFMPEG