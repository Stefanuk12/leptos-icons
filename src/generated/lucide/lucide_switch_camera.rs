use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 19H4a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h5" ></ path > < path d = "M13 5h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-5" ></ path > < circle cx = "12" cy = "12" r = "3" ></ circle > < path d = "m18 22-3-3 3-3" ></ path > < path d = "m6 2 3 3-3 3" ></ path > < / > } } pub const LucideSwitchCamera : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;