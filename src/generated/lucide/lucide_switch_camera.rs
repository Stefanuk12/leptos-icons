use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 19H4a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h5" ></ path > < path d = "M13 5h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-5" ></ path > < circle cy = "12" r = "3" cx = "12" ></ circle > < path d = "m18 22-3-3 3-3" ></ path > < path d = "m6 2 3 3-3 3" ></ path > < / > } } pub const LUCIDE_SWITCH_CAMERA : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none")] } ;