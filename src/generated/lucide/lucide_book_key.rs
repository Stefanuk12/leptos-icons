use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H14" ></ path > < path d = "M20 8v14H6.5a2.5 2.5 0 0 1 0-5H20" ></ path > < circle cx = "14" r = "2" cy = "8" ></ circle > < path d = "m20 2-4.5 4.5" ></ path > < path d = "m19 3 1 1" ></ path > < / > } } pub const LucideBookKey : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("fill" , "none")] } ;