use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "20" height = "16" y = "4" x = "2" ></ rect > < path d = "M12 9v11" ></ path > < path d = "M2 9h13a2 2 0 0 1 2 2v9" ></ path > < / > } } pub const LUCIDE_PROPORTIONS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;