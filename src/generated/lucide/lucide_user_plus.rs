use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle cx = "9" r = "4" cy = "7" ></ circle > < line y1 = "8" y2 = "14" x1 = "19" x2 = "19" ></ line > < line x2 = "16" y1 = "11" y2 = "11" x1 = "22" ></ line > < / > } } pub const LUCIDE_USER_PLUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor")] } ;