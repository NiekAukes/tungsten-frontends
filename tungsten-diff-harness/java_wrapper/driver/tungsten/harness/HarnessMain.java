package tungsten.harness;

import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.file.Files;
import java.nio.file.Paths;

import tungsten.generated.GeneratedWorldgen;
import tungsten.runtime.BlockBox;
import tungsten.runtime.Jigsaw;
import tungsten.runtime.Piece;

/**
 * Java half of the diff harness. Runs {@code orchestrate_final_density} for one chunk and
 * writes the {@value #DIMS} results as raw little-endian doubles to a file.
 *
 * Usage: HarnessMain seed origin_x origin_y origin_z output_path
 */
public final class HarnessMain {
    private static final int DIMS = 16 * 384 * 16;

    public static void main(String[] args) throws IOException {
        if (args.length != 5) {
            System.err.println("usage: HarnessMain <seed> <ox> <oy> <oz> <output>");
            System.exit(64);
        }
        long seed = Long.parseLong(args[0]);
        double ox = Double.parseDouble(args[1]);
        double oy = Double.parseDouble(args[2]);
        double oz = Double.parseDouble(args[3]);

        // Same empty host constants as the Rust wrapper. BlockBox/Piece defaults mirror
        // tungsten-mc/src/mathf64.rs (max = -1.0 marks an unused box).
        GeneratedWorldgen.HostConsts hostConsts = new GeneratedWorldgen.HostConsts();
        BlockBox emptyBox = new BlockBox(0.0, 0.0, 0.0, -1.0, -1.0, -1.0);
        hostConsts.base_box = emptyBox;
        hostConsts.jigsaw_array = new Jigsaw[500];
        hostConsts.piece_array = new Piece[500];
        for (int i = 0; i < 500; i++) {
            hostConsts.jigsaw_array[i] = new Jigsaw(0, 0, 0, 0);
            hostConsts.piece_array[i] = new Piece(emptyBox, 0, 0);
        }

        GeneratedWorldgen.AutoConsts autoConsts = GeneratedWorldgen.AutoConsts.create(seed);
        GeneratedWorldgen.Orchestration orchestration = new GeneratedWorldgen.Orchestration(autoConsts);
        double[] result = orchestration.orchestrate_final_density(ox, oy, oz, hostConsts);

        if (result.length != DIMS) {
            System.err.println("expected " + DIMS + " samples, got " + result.length);
            System.exit(2);
        }

        // putDouble keeps NaN payloads, like doubleToRawLongBits.
        ByteBuffer buf = ByteBuffer.allocate(DIMS * Double.BYTES).order(ByteOrder.LITTLE_ENDIAN);
        for (double v : result) {
            buf.putDouble(v);
        }
        Files.write(Paths.get(args[4]), buf.array());
    }
}
