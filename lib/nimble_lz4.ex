defmodule NimbleLZ4 do
  @moduledoc """
  [LZ4](https://github.com/lz4/lz4) compression and decompression.

  This functionality is built on top of native (Rust) NIFs. There is no
  streaming functionality, everything is done in memory.

  ## Uncompressed Size

  `decompress/2` takes the original uncompressed size as a parameter. For this
  reason, it's common to store compressed binaries *prefixed by their uncompressed
  length*. For example, you could store the compressed binary as:

      my_binary = <<...>>
      store(<<byte_size(my_binary)::32>> <> NimbleLZ4.compress(my_binary))

  When decompressing, you can extract the uncompressed length:

      <<uncompressed_size::32, compressed_binary::binary>> = retrieve_binary()
      uncompressed_binary = NimbleLZ4.decompress(compressed_binary, uncompressed_size)

  ## Frame Compression with File Output

  For large data that needs to be compressed directly to a file, you can use the
  resource-based frame compression functions:

      {:ok, compressor} = NimbleLZ4.create_frame_compressor_with_file_output("output.lz4")
      :ok = NimbleLZ4.write_to_frame(compressor, chunk1)
      :ok = NimbleLZ4.write_to_frame(compressor, chunk2)
      :ok = NimbleLZ4.finish_frame(compressor)

  """

  use Rustler, otp_app: :nimble_lz4, crate: "nimblelz4"

  @doc """
  Compresses the given binary.
  """
  @doc since: "0.1.0"
  @spec compress(binary()) :: binary()
  def compress(_binary) do
    :erlang.nif_error(:nif_not_loaded)
  end

  @doc """
  Decompresses the given binary using the size of the uncompressed binary.
  """
  @spec decompress(binary(), non_neg_integer()) :: {:ok, binary()} | {:error, term()}
  def decompress(_binary, _uncompressed_size) do
    :erlang.nif_error(:nif_not_loaded)
  end

  @doc """
  Compresses the given binary using the [LZ4 frame
  format](https://github.com/lz4/lz4/blob/dev/doc/lz4_Frame_format.md) into a frame.
  """
  @doc since: "1.1.0"
  @spec compress_frame(binary()) :: binary()
  def compress_frame(_binary) do
    :erlang.nif_error(:nif_not_loaded)
  end

  @doc """
  Decompresses the given frame binary using the [LZ4 frame
  format](https://github.com/lz4/lz4/blob/dev/doc/lz4_Frame_format.md).
  """
  @doc since: "1.1.0"
  @spec decompress_frame(binary()) :: {:ok, binary()} | {:error, term()}
  def decompress_frame(_binary) do
    :erlang.nif_error(:nif_not_loaded)
  end

  @doc """
  Creates a new frame compressor that writes compressed data directly to a file.

  This is useful for compressing large amounts of data without keeping everything
  in memory. The compressor must be finished with `finish_frame/1` to properly
  close the compressed file.

  ## Examples

      {:ok, compressor} = NimbleLZ4.create_frame_compressor_with_file_output("data.lz4")
      :ok = NimbleLZ4.write_to_frame(compressor, "Hello, ")
      :ok = NimbleLZ4.write_to_frame(compressor, "World!")
      :ok = NimbleLZ4.finish_frame(compressor)

  """
  @spec create_frame_compressor_with_file_output(String.t()) ::
          {:ok, reference()} | {:error, String.t()}
  def create_frame_compressor_with_file_output(_output_path) do
    :erlang.nif_error(:nif_not_loaded)
  end

  @doc """
  Writes a chunk of data to the frame compressor.

  The data will be compressed and written to the output file. This function
  can be called multiple times to write data in chunks.

  ## Examples

      {:ok, compressor} = NimbleLZ4.create_frame_compressor_with_file_output("data.lz4")
      :ok = NimbleLZ4.write_to_frame(compressor, chunk_data)

  """
  @spec write_to_frame(reference(), binary()) :: :ok | {:error, String.t()}
  def write_to_frame(_resource, _chunk) do
    :erlang.nif_error(:nif_not_loaded)
  end

  @doc """
  Finishes the frame compression and closes the output file.

  This function must be called to properly finalize the compressed file.
  After calling this function, the compressor resource cannot be used again.

  ## Examples

      {:ok, compressor} = NimbleLZ4.create_frame_compressor_with_file_output("data.lz4")
      :ok = NimbleLZ4.write_to_frame(compressor, data)
      :ok = NimbleLZ4.finish_frame(compressor)

  """
  @spec finish_frame(reference()) :: :ok | {:error, String.t()}
  def finish_frame(_resource) do
    :erlang.nif_error(:nif_not_loaded)
  end
end