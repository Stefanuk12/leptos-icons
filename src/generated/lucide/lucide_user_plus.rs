use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle cy = "7" r = "4" cx = "9" ></ circle > < line x1 = "19" x2 = "19" y1 = "8" y2 = "14" ></ line > < line y1 = "11" y2 = "11" x2 = "16" x1 = "22" ></ line > < / > } } pub const LUCIDE_USER_PLUS : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;