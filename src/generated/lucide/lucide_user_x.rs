use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle r = "4" cx = "9" cy = "7" ></ circle > < line y1 = "8" y2 = "13" x2 = "22" x1 = "17" ></ line > < line x1 = "22" x2 = "17" y1 = "8" y2 = "13" ></ line > < / > } } pub const LUCIDE_USER_X : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;