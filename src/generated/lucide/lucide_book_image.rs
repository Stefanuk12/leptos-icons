use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" ></ path > < circle cy = "8" r = "2" cx = "10" ></ circle > < path d = "m20 13.7-2.1-2.1c-.8-.8-2-.8-2.8 0L9.7 17" ></ path > < / > } } pub const LUCIDE_BOOK_IMAGE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;