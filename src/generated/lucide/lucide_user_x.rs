use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle cx = "9" r = "4" cy = "7" ></ circle > < line y1 = "8" x1 = "17" y2 = "13" x2 = "22" ></ line > < line y1 = "8" x2 = "17" x1 = "22" y2 = "13" ></ line > < / > } } pub const LUCIDE_USER_X : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;