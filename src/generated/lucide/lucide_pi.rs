use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 7c0-1.7 1.3-3 3-3h13" ></ path > < path d = "M18 20c-1.7 0-3-1.3-3-3V4" ></ path > < / > } } pub const LucidePi : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;