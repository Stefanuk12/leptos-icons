use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 21a6 6 0 0 0-12 0" ></ path > < circle cy = "11" r = "4" cx = "12" ></ circle > < rect height = "18" x = "3" rx = "2" width = "18" y = "3" ></ rect > < / > } } pub const LUCIDE_SQUARE_USER_ROUND : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;