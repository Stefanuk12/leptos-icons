use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" rx = "2" x = "3" height = "18" y = "3" ></ rect > < path d = "M12 9v6" ></ path > < path d = "M16 15v6" ></ path > < path d = "M16 3v6" ></ path > < path d = "M3 15h18" ></ path > < path d = "M3 9h18" ></ path > < path d = "M8 15v6" ></ path > < path d = "M8 3v6" ></ path > < / > } } pub const LUCIDE_BRICK_WALL : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;