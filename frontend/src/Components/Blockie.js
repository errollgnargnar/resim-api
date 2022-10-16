import Blockies from 'react-blockies';
 
export const Blockie = ({seed}) => (
  <Blockies
    seed={seed}/* the only required prop; determines how the image is generated */
    size={10} /* number of squares wide/tall the image will be; default = 15 */
    scale={3} /* width/height of each square in pixels; default = 4 */
    color="#fff" /* normal color; random by default */
    bgColor="#000" /* background color; random by default */
    spotColor="#abc" /* color of the more notable features; random by default */
    className="identicon" /* optional class name for the canvas element; "identicon" by default */
  />
)

export default Blockie;