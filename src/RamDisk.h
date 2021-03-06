#pragma once

#include <fastspd.h>

#include <array>
#include <unordered_map>

namespace L4
{
    class RamDisk : public FastSpd::VirtualDisk
    {
    public:
        static constexpr uint64_t BlockSize = 1 << 12;
        static constexpr uint64_t DiskSize = 1ll << 38;

        static constexpr uint64_t BlockCount = DiskSize / BlockSize;
        static_assert(DiskSize % BlockSize == 0, "Disk size must be a multiple of block size");

        RamDisk();

        void Read(std::byte* Buffer, uint64_t BlockAddress, uint32_t BlockCount) noexcept override;

        void Write(const std::byte* Buffer, uint64_t BlockAddress, uint32_t BlockCount) noexcept override;

        void Flush(uint64_t BlockAddress, uint32_t BlockCount) noexcept override;

        void Unmap(uint64_t BlockAddress, uint32_t BlockCount) noexcept override;

    private:
        std::unordered_map<uint64_t, std::array<std::byte, BlockSize>> Disk;
    };
}