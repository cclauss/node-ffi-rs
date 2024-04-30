const Koa = require('koa');
const app = new Koa();
process.env.MEMORY = 1
const { unitTest } = require('./test')

app.use(async ctx => {
  console.log('memory:', (process.memoryUsage().heapUsed / 1024 / 1024).toFixed(2))
  unitTest()
  ctx.body = 'success'
});

app.listen(3000);
