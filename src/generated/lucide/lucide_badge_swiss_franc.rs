use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" ></ path > < path d = "M11 17V8h4" ></ path > < path d = "M11 12h3" ></ path > < path d = "M9 16h4" ></ path > < / > } } pub const LucideBadgeSwissFranc : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2")] } ;