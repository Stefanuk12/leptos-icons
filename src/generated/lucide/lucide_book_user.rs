use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" ></ path > < circle r = "2" cy = "8" cx = "12" ></ circle > < path d = "M15 13a3 3 0 1 0-6 0" ></ path > < / > } } pub const LUCIDE_BOOK_USER : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round")] } ;