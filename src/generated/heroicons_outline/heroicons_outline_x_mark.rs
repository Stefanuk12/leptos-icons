use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 18 18 6M6 6l12 12" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_X_MARK : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;