use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 21a6 6 0 0 0-12 0" ></ path > < circle cy = "11" cx = "12" r = "4" ></ circle > < rect y = "3" x = "3" rx = "2" width = "18" height = "18" ></ rect > < / > } } pub const LUCIDE_SQUARE_USER_ROUND : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor")] } ;