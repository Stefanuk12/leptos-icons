use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 19H4a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h5" ></ path > < path d = "M13 5h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-5" ></ path > < circle r = "3" cy = "12" cx = "12" ></ circle > < path d = "m18 22-3-3 3-3" ></ path > < path d = "m6 2 3 3-3 3" ></ path > < / > } } pub const LucideSwitchCamera : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;