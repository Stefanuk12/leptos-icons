use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" width = "20" height = "16" rx = "2" x = "2" ></ rect > < path d = "M12 9v11" ></ path > < path d = "M2 9h13a2 2 0 0 1 2 2v9" ></ path > < / > } } pub const LUCIDE_PROPORTIONS : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;