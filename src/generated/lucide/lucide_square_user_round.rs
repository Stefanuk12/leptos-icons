use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 21a6 6 0 0 0-12 0" ></ path > < circle r = "4" cy = "11" cx = "12" ></ circle > < rect rx = "2" x = "3" width = "18" y = "3" height = "18" ></ rect > < / > } } pub const LUCIDE_SQUARE_USER_ROUND : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24")] } ;