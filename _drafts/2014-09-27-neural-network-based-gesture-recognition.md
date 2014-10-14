---
layout: post
title: 'Neural network-based gesture recognition with Leap Motion'
date: 2014-09-27 11:30
---

This post is the translation of a [paper][pdf] written by me and Giovanni Pinto as an essay for the Neural Networks exam, taught by Gregorio Landi at UniFi. We studied the treatment and classification of Leap Motion data via artificial neural networks, and projected a gesture recognition system.

## Table of contents

1. Introduction
2. The problem and the involved technologies
  1. Acquisition and data structures
  2. Filtering an useful subset of information
  3. Design and algorithms for the recognition software
3. Neural networks for gesture recognition
  1. What is a gesture
  2. Issues with dynamic gesture recognition
  3. Gesture alphabet
  4. Network design
4. Performance analysis
  1. Training sample number
  2. Recognizing hands by different subjects
5. Conclusions

## 1. Introduction

In the last few years, research and development in User Experience led to a radical change in interaction paradigms. An excellent example of this phenomenon is represented by the introduction of smartphones and touch-screen tablet computers, whose ease of use and accessibility noticeably lowered the barriers to entry in the computing world. At the same time, engineering departments of enterprises both small and large are moving on to experiment on unusual and counterintuitive paradigms: Oculus VR produces an head-mounted display widening the field of vision allowing virtual reality application, Google leads experiments on high-powered portable devices for 3D environmental mapping (Project Tango), developer communities invent algorithms for motion recognition with instruments such as Microsoft's Kinect.

Among those projects is the one by U.S. based Leap Motion: a hand motion detection device originally conceived to simplify 3D modeling and sculpting, making it intuitive as in the real world. Leap detects, in an approximately demispherical spatial area, obstacles distant up to about a meter from its surface. Two monochromatic cameras collect infrared light reflected from three LEDs onto the obstacles' surface and computer vision algorithms, not disclosed by the company, calculate the hands' position and give a standardized 3D euclidean space representation.

Data supplied by Leap Motion are useful in contexts where instantaneous spatial position is an important element of the interaction, eg. the manipulation of objects in euclidean space; an application which isn't contemplated among the devices' standard features is gesture semantics. Via the arguments treated in the present paper, we want to offer a system which allows to classify a generic symbol alphabet, whose applications will then vary with the context where the system is deployed. The context we wanted to study is about sampling and classifying [American sign language][asl]. Plausible applications on this domain may be the transcription of conversations between deaf-mute people, or writing in contexts where the usage of a full keyboard might prove uncomfortable (for example on very small devices or in tight spaces, such as a mine or underwater). We will see why and how neural networks are an useful instrument in treating this specific kind of problem.


## 2. The problem and the involved technologies

### 2.1 Acquisition and data structures

<img src='/data/2014-09-27/mano.svg' style='width: 40%; max-width: 40rem; float: left; margin-right: 1rem; margin-bottom: 1rem; background-color: white; '/> A recent beta release of Leap's firmware made it possible to encode the hand as a hierarchical structure similar to the one naturally present in the hand's bones: the palm is the root of the tree, metacarps are first-level children (on the thumb, which doesn't really have one, the metacarp data structure has null length). Each metacarpal phalanx has one and only one child node, proximal phalanx, which will extend in a node representing the intermediate phalanx, which will terminate in a leaf, the distal phalanx (see picture). This structure is similar, both in the concept and in the encoding, to the one used in [3D computer graphics animation](2014/05/09/implementing-skeletal-animation.html). Each phalanx, in fact, contains various spatial data that allow it to be effectively positioned in Leap's euclidean space: a vector representing its translation with respect to the palm, a normal vector encoding its direction, a scalar representing the length, an orthonormal basis that encodes its rotation and more; the palm encodes similar data, among which are the speed, direction, a vector normal to its surface and so on.  <i style='clear: both'></i>

### 2.2 Filtering an useful subset of information

We want to train a neural network to recognize gestures, where a gesture is thought of as the position all joints have at a certain time. Not all the data Leap supplies are useful to this purpose, and could even prove counterproductive. If, for example, we considered the absolute spatial positioning of each joint, we would have to train the network with a very very large number of positions of the hand in space, and for each of those a very very large number of hands of different sizes, since the absolute position of each joint could vary significantly with the size of the hand. Besides, in the way a human describes a gesture, spatial position is a factor which doesn't contribute to its conceptual definition: when the wise points at the moon, the fool distinguishes the gesture made when the wise man is by his side and the gesture made when the wise man is on the other side of the street. It is important thus to search for a subset of the parameters supplied by Leap which describes accurately and as independently and abstractly as possible the concept of gesture that we want to recognize.

A useful parameter to represent the gesture is the way the articulations are rotated in the space. Each rotation is representable via a normal vector which simply points to the phalanx' direction. Geometrically it is calculated by translating the joint to the center of the space and normalizing the vector which goes from the center to the next joint, or by normalizing the vector calculated as the difference between the next joint's position and the current joint's position. We will then just record the surface normal to the palm. In reality, obviously, the palm doesn't have a purely planar surface, but the Leap supplies a semantically meaningful approximation. Palm's surface normal represents, in the scope of the project, the same conceptual function of each of the phalanxes, which is to represent syntetically a rotation.

We decide thus to represent the layout of the features plugged on the input of the neural networks as the concatenation of the palm surface normal and all the normalized direction vectors of the fingers' joints, following this scheme:

$$
\begin{array}{lll}
[palmNormal_x, & palmNormal_y, & palmNormal_z,\\
thumb_x^0, & thumb_y^0, & thumb_z^0,\\
thumb_x^1, & thumb_y^1, & thumb_z^1,\\
thumb_x^2, & thumb_y^2, & thumb_z^2,\\
& \ldots\\
little_x^3, & little_y^3, & little_z^3 ]\\
\end{array}
$$

where the subscript represents the axis of the vector, and the superscript represents the phalanx, according to the following scheme:

$$
\begin{array}{ll}
0 & metacarpal\\
1 & proximal\\
2 & intermediate\\
3 & distal
\end{array}
$$

[pdf]: #
[asl]: http://en.wikipedia.org/wiki/American_Sign_Language
