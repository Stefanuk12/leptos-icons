use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "2" y = "4" height = "16" width = "20" ></ rect > < path d = "M12 9v11" ></ path > < path d = "M2 9h13a2 2 0 0 1 2 2v9" ></ path > < / > } } pub const LUCIDE_PROPORTIONS : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;