<template>
    <div class="fixed bottom-21.2 inset-x-0 z-999999999">
        <div class="loader"></div>
    </div>
</template>

<style>
.loader {
    width: 566px;
    height: 2px;
    display: inline-block;
    position: relative;
    background: rgba(255, 255, 255, 0.15);
    overflow: hidden;
}

.loader::after {
    content: '';
    width: 100px;
    height: 2px;
    background: #fff;
    position: absolute;
    top: 0;
    left: 0;
    box-sizing: border-box;
    animation: animloader 1s linear infinite;
}

@keyframes animloader {
    0% {
        left: 0;
        transform: translateX(-100%);
    }

    100% {
        left: 100%;
        transform: translateX(0%);
    }
}
</style>