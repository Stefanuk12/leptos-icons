use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" rx = "2" y = "3" x = "3" ></ rect > < path d = "M3 9h18" ></ path > < path d = "M9 21V9" ></ path > < / > } } pub const LucidePanelsTopLeft : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;