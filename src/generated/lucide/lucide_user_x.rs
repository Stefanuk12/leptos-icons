use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle cy = "7" cx = "9" r = "4" ></ circle > < line y1 = "8" y2 = "13" x2 = "22" x1 = "17" ></ line > < line x1 = "22" y1 = "8" x2 = "17" y2 = "13" ></ line > < / > } } pub const LUCIDE_USER_X : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none")] } ;