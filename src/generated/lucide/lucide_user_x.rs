use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle r = "4" cx = "9" cy = "7" ></ circle > < line y2 = "13" x2 = "22" x1 = "17" y1 = "8" ></ line > < line x2 = "17" x1 = "22" y2 = "13" y1 = "8" ></ line > < / > } } pub const LUCIDE_USER_X : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;