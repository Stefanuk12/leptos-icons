use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3v18" ></ path > < rect rx = "2" width = "18" x = "3" height = "18" y = "3" ></ rect > < path d = "M3 9h18" ></ path > < path d = "M3 15h18" ></ path > < / > } } pub const LUCIDE_TABLE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;