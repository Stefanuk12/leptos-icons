use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle r = "4" cx = "9" cy = "7" ></ circle > < line x1 = "17" x2 = "22" y2 = "13" y1 = "8" ></ line > < line y1 = "8" y2 = "13" x2 = "17" x1 = "22" ></ line > < / > } } pub const LUCIDE_USER_X : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;