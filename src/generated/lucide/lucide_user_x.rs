use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle cx = "9" cy = "7" r = "4" ></ circle > < line x1 = "17" y1 = "8" y2 = "13" x2 = "22" ></ line > < line y1 = "8" x1 = "22" y2 = "13" x2 = "17" ></ line > < / > } } pub const LucideUserX : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;