use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle r = "4" cy = "7" cx = "9" ></ circle > < line x2 = "22" y2 = "13" y1 = "8" x1 = "17" ></ line > < line y1 = "8" y2 = "13" x1 = "22" x2 = "17" ></ line > < / > } } pub const LUCIDE_USER_X : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;