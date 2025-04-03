import init, { CanvasParameters } from "../../../pkg/srkimgedit.js";

await init();

let obj = new CanvasParameters();
const path = "/img/croptesting.jpg";
document.getElementById("container").appendChild(obj.get_canvas());
obj.draw_image(path, true, false);
// document.getElementById("crop").addEventListener("click", () => {
//   obj.applyPlugin("crop", {
//     canvasToApplyCrop: obj.getCanvas(),
//     imagePathToApplyCrop: path,
//     isCropStyleDocument: true,
//   });
// });
// document.getElementById("cut").addEventListener("click", () => {
//   obj.applyPlugin("cut");
// });
// document.getElementById("download").addEventListener("click", () => {
//   const date = new Date();
//   obj.downloadImage(date.toISOString());
// });
// document.getElementById("testplugins").addEventListener("click", () => {
//   console.log(obj.getAvailablePlugins());
//   console.log(obj.getPluginParameter("crop"));
//   console.log(obj.getPluginParameter("cut"));
// });
