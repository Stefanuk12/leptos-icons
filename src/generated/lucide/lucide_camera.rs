use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z" ></ path > < circle cx = "12" r = "3" cy = "13" ></ circle > < / > } } pub const LucideCamera : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;