use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "18" y = "3" width = "18" x = "3" ></ rect > < path d = "M12 9v6" ></ path > < path d = "M16 15v6" ></ path > < path d = "M16 3v6" ></ path > < path d = "M3 15h18" ></ path > < path d = "M3 9h18" ></ path > < path d = "M8 15v6" ></ path > < path d = "M8 3v6" ></ path > < / > } } pub const LUCIDE_BRICK_WALL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;