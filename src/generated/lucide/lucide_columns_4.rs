use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" rx = "2" height = "18" x = "3" ></ rect > < path d = "M7.5 3v18" ></ path > < path d = "M12 3v18" ></ path > < path d = "M16.5 3v18" ></ path > < / > } } pub const LUCIDE_COLUMNS_4 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;