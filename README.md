# Bevy Jam 5 - Cycles

My workspace for the [Bevy Jam 5](https://itch.io/jam/bevy-jam-5), but we'll see how far along we get. I make no promises.

## Goals
- To make a simple game
- To integrate lua scripting in a minor way

## Brainstorm
My initial idea was to create some sort of simple economic simulation taking place in a solar system where planetary orbits matter. However, in order to get something like this done in a reasonable time frame, especially with my constraints it will need to be dead simple. An inspiration of mine that does simplistic economic simulation justice is the game [Slipways](https://store.steampowered.com/app/1264280/Slipways/) which I believe was also born out of a game jam (using pico-8!).

One way to make planetary orbits matter mechanically is for any routes established between planets to care about changing distance, otherwise the planet positions might as well be static. Some ways to make distance matter are as follows:

- The cost of establishing or maintaining the route changes.
- Ships may have an effective range and sometimes may not be able to make long trips.
- Planets themselves could act as vessels, meaning resources could be stockpiled on a planet until it is close to a target.
- The cost of goods only available from one planet would fluctuate based on distance.
- Actions could be taken on board the ship in transit which require specific travel times. For example, a refinery ship might convert one resource to another en route and require a travel time of 2 cycles to complete.

In order to make trade necessary, resources need to be scarce and/or exclusive to certain planets or stations.

### Keep It Simple
I want to limit the number of interacting systems in this game so that it would be possible to implement. I also want to limit the amount of UI and art needed for the game as I will have little time to implement either.

Would it be possible to limit the game to one resource? Something like a generic "goods" resource. If we pursue a delivery company aesthetic then this single resource would make sense.

### Objectives

What is the objective of the game? To create a prosperous civilization? To make a large sum of money as some sort of transport corporation? You could be the logistics department of some large company, continually receiving job orders to transport cargo. This creates a potential fail state if too many orders go unfulfilled for too long. The puzzle of the game would be using a limited number of ships to establish routes in order to get your packages to their destinations. In some ways this idea reminds me a lot of [Mini Metro](https://store.steampowered.com/app/287980/Mini_Metro/) but where the routes are point to point and the destinations move.

### The Board
Step one will be to make the board, in this case the board will be composed of some central object (eg. The sun) surrounded by several satellites (eg. planets). I'd like to keep the terms generic because it leaves open the door for having additional scales of gameplay; Planetary system, solar system, local cluster, etc. But for the purposes of this jam we'll be focusing on a single scale.

Each satellite will produce and consume resources.

### Moment To Moment

Clicking a satellite (with a ship?) will start to establish a route, dragging to another satellite will confirm the route. A ship will then start transferring cargo to the destination and will automatically return with cargo and repeat so long as the route remains open.

Clicking the route will cancel the route, allowing the ship to finish and stop at the destination satellite.

Routes have a max range and will automatically cancel themselves if they would attempt to fly to an invalid destination. You want to avoid getting your ships stranded on a satellite while its too far away for travel!

### I Think We've Got It - In Summary
For this jam I will make a simple resource management game inspired by the likes of [Slipways](https://store.steampowered.com/app/1264280/Slipways/) and [Mini Metro](https://store.steampowered.com/app/287980/Mini_Metro/) where you manage the assignment of ships owned by a large stellar transport corporation. You have to puzzle out the best way to transport goods from start to destination using ships with a limited range flying between planets that are constantly moving relative to one another. If ever you fail to live up to corporate standards... you lose!

## 7/21/24 - Beginnings
The first step is to make the board. I am going to set up a simple planetary system made from circles traveling in circular orbits around a large central circle. Ideally I will be able to fit the game entirely on to one screen. It would be nice if there was no need to handle camera movement or zoom.

## 7/22/24 - Some Images And Custom Components
I've added [Bevy Vector Shapes](https://github.com/james-j-obrien/bevy_vector_shapes) to the project and used it to draw a simple sun and some basic orbitals. I also have begun removing demo code that is of no use to me and replacing it with my own components. Its taking a bit of work to remember how Bevy likes to do its components and queries but I can already tell that Bevy 0.14 is much improved over Bevy 0.10.

I now have the ability to add any number of planets with configurable radii and orbital speeds. Right now all orbits are perfectly circular and for the jam that is not going to change. It would be nice to one day have odd elliptical orbits to mix things up, but for now we will keep things simple.

## 7/23/24 - Planets And Interaction

There is now simple planet rendering and an automatic calculation of the arc length for the orbit around the planet. A system to handle getting the mouse position is implemented but nothing yet uses it. Soon that will be used to see if a planet has been clicked on!

I looked at how buttons work from the UI system to understand one of the options for doing interactions in a more ECS way than I was originally attempting and it is starting to make sense. Now I have a SatelliteInteraction component that makes it easy to respond to changes in state. For now I just change colors when planets are hovered or pressed with the mouse.