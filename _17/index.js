const f1 = async () => {
    console.log('f1-1');
    await Promise.resolve();
    console.log('f1-2');
    await new Promise(resolve => setTimeout(resolve, 0));
    console.log('f1-3');
}

const f2 = () => {
    console.log('f2-1');
    setTimeout(() => {
        console.log('f2-2');
    }, 0);
}

console.log('1');
f1();
f2();
console.log('2');