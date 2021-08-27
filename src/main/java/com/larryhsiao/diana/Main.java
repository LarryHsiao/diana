package com.larryhsiao.diana;

import org.takes.facets.fork.FkRegex;
import org.takes.facets.fork.TkFork;
import org.takes.http.Exit;
import org.takes.http.FtCli;

import java.io.IOException;

/**
 * Entry point of Diana.
 */
public class Main {
    public static void main(String[] args) throws IOException {
        // @todo #1 main
        new FtCli(
            new TkFork(
                new FkRegex("/", "Diana")
            ),
            args
        ).start(Exit.NEVER);
    }
}