---
title: Process your build
---

# Run your service
The Habitat depot can contain multiple versions and releases of a given artifact. When you start a Habitat service, and depending on whether or not you specify a particular version number, Habitat will either pull the latest version, or the latest release for that particular version of the artifact.

A Supervisor manages any client app or service process within a Habitat service, so when a Habitat service starts up, the Supervisor executes a command on the client software through a run script that was either generated by Habitat when the artifact was built (through the `pkg_binaries_run` setting), or through a run hook like in the case of the _mytutorialapp_.

Services can currently be run in two different ways: You can either use the `hab-sup` binary directly, or you can use a Docker container that calls the `hab-sup` binary. The second method does not require you to have the Habitat binaries built and installed on your machine.

We will show you how to run your service locally from a Docker container because this process allows you to rapidly test out changes and verify that your application works correctly before you upload your artifact to Habitat's public depot.

To create a Docker container for your artifact, perform the following steps:

1. If you are not in the studio environment, enter it from the dev shell container with the `hab-studio enter` command.
2. Change directory to the `/src/plans` directory.

        [15][default:/src:0]$cd plans

3. Build your version of the mytutorialapp artifact.

        [16][default:/src/plans:0]$build mytutorialapp

4. Run `hab install core/hab-pkg-dockerize` to unpack and install the artifact that creates docker images for other Habitat artifacts.

        [17][default:/src/plans:0]$hab install core/hab-pkg-dockerize

5. Run `hab-bpm exec core/hab-pkg-dockerize hab-pkg-dockerize origin/packagename` with the origin and name of your artifact. These values are referenced in the pkg_origin and pkg_name settings of your plan, respectively.

        [18][default:/src:0]$hab-bpm exec core/hab-pkg-dockerize hab-pkg-dockerize myorigin/mytutorialapp

    Habitat will proceed to unpack and install all necessary Habitat artifacts, the Habitat command-line interface (CLI) tools and binaries, the mytutorialapp artifact, and all of its dependencies. Then it will create an image using the Docker scratch image as the base image and build up the rest of the image from there.

    Once that process has completed, you can run your Docker image inside a container from any terminal window that has access to the Docker CLI, such as the dev shell container or your host machine.

      > Note: We have to publish the Docker container port number to allow that container to be accessed by the host machine. Also, you may need to connect your shell to the machine using `eval "$(docker-machine env default)"`

        $ docker run -it -p 8080:8080 myorigin/mytutorialapp
        hab-sup(MN): Starting myorigin/mytutorialapp
        hab-sup(GS): Supervisor 172.17.0.3: 10293333-7317-4c79-9a2f-2ca04e340727
        hab-sup(GS): Census mytutorialapp.default: ccc084da-03ca-46af-9bb1-edc5942a9f0a
        hab-sup(GS): Starting inbound gossip listener
        hab-sup(GS): Starting outbound gossip distributor
        hab-sup(GS): Starting gossip failure detector
        hab-sup(CN): Starting census health adjuster
        hab-sup(SC): Updated config.json
        hab-sup(TP): Restarting because the service config was updated via the census
        mytutorialapp(SV): Starting
        mytutorialapp(O):
        mytutorialapp(O): > nodejs_tutorial_app@0.1.0 start /hab/svc/mytutorialapp
        mytutorialapp(O): > node server.js
        mytutorialapp(O):
        mytutorialapp(O): Running on http://0.0.0.0:8080

4. Because we are running this service in a Docker container, you need the IP address of the VM that is running it to access the message of the day. Type the following command in your terminal window to bring up the site:

        open "http://$(docker-machine ip default):8080/"

    Here's an example of what you should see in your browser:

    ![Screen shot of node.js tutorial output](/images/nodejs-tutorial-output.png)

5. Finally, because of the configuration capabilities in Habitat, you can now re-run your docker container and update the message value when your Habitat service starts up. To do this, you must pass in a Docker environment variable with the following format: `HAB_PACKAGENAME='keyname=newvalue'`.

    > Note: The package name in the environment variable must be uppercase and any dashes must be replaced with underscores.

    Here is how you change the message for _mytutorialapp_:


        $ docker run -e HAB_MYTUTORIALAPP='message="Habitat rocks!"' -p 8080:8080 -it myorigin/mytutorialapp
        hab-sup(MN): Starting sample/mytutorialapp
        hab-sup(GS): Supervisor 172.17.0.3: 70c03728-4daa-49fe-b67b-3e0426bb6037
        hab-sup(GS): Census mytutorialapp.default: bc2f3ec8-6212-490e-afca-e2d8c18013c3
        hab-sup(GS): Starting inbound gossip listener
        hab-sup(GS): Starting outbound gossip distributor
        hab-sup(GS): Starting gossip failure detector
        hab-sup(CN): Starting census health adjuster
        hab-sup(SC): Updated config.json
        hab-sup(TP): Restarting because the service config was updated via the census
        mytutorialapp(SV): Starting
        mytutorialapp(O):
        mytutorialapp(O): > nodejs_tutorial_app@0.1.0 start /hab/svc/mytutorialapp
        mytutorialapp(O): > node server.js
        mytutorialapp(O):
        mytutorialapp(O): Running on http://0.0.0.0:8080


    Now refresh, or connect again to the local URL through your web browser.

    ![Screen shot of node.js output with new message](/images/nodejs-tutorial-update-output.png)

Congratulations, you have successfully built a package from scratch, run it locally, and tested out some of the configuration capabilities!

Now that you have played around with the basics of Habitat, go over to the Docs section of the site to learn more about plans, configuring client software, how communication works between Habitat services, and so on.

<hr>
<ul class="main-content--button-nav">
  <li><a href="/tutorials/getting-started-next-steps" class="button cta">Next Steps</a></li>
  <li><a href="/tutorials/getting-started-configure-plan/">Back to previous step</a></li>
</ul>