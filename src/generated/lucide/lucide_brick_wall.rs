use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "18" y = "3" x = "3" height = "18" ></ rect > < path d = "M12 9v6" ></ path > < path d = "M16 15v6" ></ path > < path d = "M16 3v6" ></ path > < path d = "M3 15h18" ></ path > < path d = "M3 9h18" ></ path > < path d = "M8 15v6" ></ path > < path d = "M8 3v6" ></ path > < / > } } pub const LUCIDE_BRICK_WALL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;