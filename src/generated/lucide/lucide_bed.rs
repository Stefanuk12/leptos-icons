use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 4v16" ></ path > < path d = "M2 8h18a2 2 0 0 1 2 2v10" ></ path > < path d = "M2 17h20" ></ path > < path d = "M6 8v9" ></ path > < / > } } pub const LucideBed : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;