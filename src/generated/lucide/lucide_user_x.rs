use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle cy = "7" r = "4" cx = "9" ></ circle > < line x2 = "22" x1 = "17" y1 = "8" y2 = "13" ></ line > < line y1 = "8" x2 = "17" y2 = "13" x1 = "22" ></ line > < / > } } pub const LUCIDE_USER_X : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;