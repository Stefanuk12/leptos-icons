use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" height = "18" x = "3" width = "18" ry = "2" ></ rect > < path d = "M9 17c2 0 2.8-1 2.8-2.8V10c0-2 1-3.3 3.2-3" ></ path > < path d = "M9 11.2h5.7" ></ path > < / > } } pub const LUCIDE_SQUARE_FUNCTION : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24")] } ;