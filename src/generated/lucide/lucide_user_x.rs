use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle cx = "9" r = "4" cy = "7" ></ circle > < line x2 = "22" y1 = "8" x1 = "17" y2 = "13" ></ line > < line y2 = "13" x1 = "22" y1 = "8" x2 = "17" ></ line > < / > } } pub const LUCIDE_USER_X : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;