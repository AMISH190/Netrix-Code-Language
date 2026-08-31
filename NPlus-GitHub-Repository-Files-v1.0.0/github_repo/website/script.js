document.getElementById('year').textContent = new Date().getFullYear();

const messages = [
  'nplus run examples/hello.npl',
  'nplus check examples/hello.npl',
  'nplus repl'
];
let idx = 0;
const typed = document.getElementById('typed');
setInterval(() => {
  idx = (idx + 1) % messages.length;
  typed.style.opacity = '0';
  setTimeout(() => {
    typed.textContent = messages[idx];
    typed.style.opacity = '1';
  }, 180);
}, 3200);

typed.style.transition = 'opacity .18s ease';
