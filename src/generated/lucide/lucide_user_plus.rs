use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle r = "4" cx = "9" cy = "7" ></ circle > < line x1 = "19" y2 = "14" y1 = "8" x2 = "19" ></ line > < line x2 = "16" y2 = "11" x1 = "22" y1 = "11" ></ line > < / > } } pub const LUCIDE_USER_PLUS : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;