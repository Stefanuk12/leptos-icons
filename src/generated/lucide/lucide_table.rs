use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3v18" ></ path > < rect width = "18" rx = "2" x = "3" height = "18" y = "3" ></ rect > < path d = "M3 9h18" ></ path > < path d = "M3 15h18" ></ path > < / > } } pub const LUCIDE_TABLE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;