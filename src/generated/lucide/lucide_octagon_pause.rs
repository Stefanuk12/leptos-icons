use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 15V9" ></ path > < path d = "M14 15V9" ></ path > < path d = "M7.714 2h8.572L22 7.714v8.572L16.286 22H7.714L2 16.286V7.714z" ></ path > < / > } } pub const LUCIDE_OCTAGON_PAUSE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;