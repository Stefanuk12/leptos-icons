use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" height = "16" x = "2" y = "4" rx = "2" ></ rect > < path d = "M12 9v11" ></ path > < path d = "M2 9h13a2 2 0 0 1 2 2v9" ></ path > < / > } } pub const LUCIDE_PROPORTIONS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;