use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle cy = "7" r = "4" cx = "9" ></ circle > < line x2 = "19" x1 = "19" y1 = "8" y2 = "14" ></ line > < line y2 = "11" x1 = "22" x2 = "16" y1 = "11" ></ line > < / > } } pub const LUCIDE_USER_PLUS : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24")] } ;