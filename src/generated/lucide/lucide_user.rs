use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" ></ path > < circle cx = "12" cy = "7" r = "4" ></ circle > < / > } } pub const LUCIDE_USER : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;