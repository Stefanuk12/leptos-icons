use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" width = "18" height = "18" rx = "2" ry = "2" x = "3" ></ rect > < path d = "M9 17c2 0 2.8-1 2.8-2.8V10c0-2 1-3.3 3.2-3" ></ path > < path d = "M9 11.2h5.7" ></ path > < / > } } pub const LUCIDE_SQUARE_FUNCTION : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;