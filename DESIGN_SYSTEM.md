# Design System: Brutalist Industrial UI
## Zero Border-Radius Design System

---

## 1. FILOZOFIA DESIGNU

### 1.1 Zasady Podstawowe
- **ZERO BORDER-RADIUS**: Wszystkie elementy mają ostre krawędzie 90°
- **Brutalism Industrial**: Funkcjonalność przed estetyką
- **Grid-Based Layout**: Wszystko oparte na siatce 4px/8px
- **High Contrast**: WCAG AAA compliance
- **Precision**: Każdy piksel ma cel

### 1.2 Inspiracje
- Brutalist architecture
- Industrial design
- Enterprise software (Terminal, Vim, Emacs)
- Technical documentation
- Military interfaces

---

## 2. DESIGN TOKENS

### 2.1 Kolory (Color Palette)

#### Background Colors
```
--color-bg-primary:     #000000    /* Główny background */
--color-bg-secondary:   #0a0a0a    /* Sekcje, karty */
--color-bg-tertiary:    #1a1a1a    /* Inputy, przyciski */
--color-bg-elevated:    #252525    /* Modale, dropdowny */
--color-bg-hover:       rgba(255, 255, 255, 0.08)  /* Hover states */
--color-bg-active:      rgba(255, 255, 255, 0.15)   /* Active/Selected */
--color-bg-focus:       rgba(255, 255, 255, 0.12)   /* Focus ring */
```

#### Foreground Colors
```
--color-fg-primary:     #ffffff    /* Główny tekst */
--color-fg-secondary:   #cccccc    /* Tekst drugorzędny */
--color-fg-tertiary:    #999999    /* Tekst trzeciorzędny */
--color-fg-disabled:    #666666    /* Wyłączone elementy */
--color-fg-accent:      #ffffff    /* Akcenty, linki */
```

#### Border Colors
```
--color-border-primary:   #333333    /* Główne obramowania */
--color-border-secondary: #444444    /* Wtórne obramowania */
--color-border-focus:     #ffffff    /* Focus borders */
--color-border-error:     #ff4444    /* Błędy */
--color-border-success:   #44ff44    /* Sukces */
--color-border-warning:   #ffaa00    /* Ostrzeżenia */
```

#### Semantic Colors
```
--color-error-bg:      rgba(255, 68, 68, 0.15)
--color-error-fg:      #ff4444
--color-success-bg:   rgba(68, 255, 68, 0.15)
--color-success-fg:    #44ff44
--color-warning-bg:    rgba(255, 170, 0, 0.15)
--color-warning-fg:    #ffaa00
--color-info-bg:       rgba(68, 170, 255, 0.15)
--color-info-fg:        #44aaff
```

### 2.2 Typografia (Typography)

#### Font Stack
```
--font-family-primary:   'Inter', 'SF Pro Display', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif
--font-family-mono:      'JetBrains Mono', 'Fira Code', 'Consolas', 'Monaco', monospace
```

#### Font Sizes (Scale: 1.25)
```
--font-size-xs:      10px    /* 0.625rem */
--font-size-sm:      12px    /* 0.75rem */
--font-size-base:    14px    /* 0.875rem */
--font-size-md:      16px    /* 1rem */
--font-size-lg:      20px    /* 1.25rem */
--font-size-xl:      24px    /* 1.5rem */
--font-size-2xl:     32px    /* 2rem */
--font-size-3xl:     40px    /* 2.5rem */
```

#### Font Weights
```
--font-weight-light:     300
--font-weight-regular:  400
--font-weight-medium:   500
--font-weight-semibold: 600
--font-weight-bold:     700
```

#### Line Heights
```
--line-height-tight:   1.2
--line-height-normal:  1.5
--line-height-relaxed: 1.75
```

#### Letter Spacing
```
--letter-spacing-tight:  -0.02em
--letter-spacing-normal:  0
--letter-spacing-wide:    0.05em
```

### 2.3 Spacing (8px Grid System)

```
--spacing-0:   0px
--spacing-1:   4px    /* 0.25rem */
--spacing-2:   8px    /* 0.5rem */
--spacing-3:   12px   /* 0.75rem */
--spacing-4:   16px   /* 1rem */
--spacing-5:   20px   /* 1.25rem */
--spacing-6:   24px   /* 1.5rem */
--spacing-8:   32px   /* 2rem */
--spacing-10:  40px   /* 2.5rem */
--spacing-12:  48px   /* 3rem */
--spacing-16:  64px   /* 4rem */
--spacing-20:  80px   /* 5rem */
```

### 2.4 Borders

```
--border-width-none:   0px
--border-width-thin:   1px
--border-width-medium: 2px
--border-width-thick:  3px
--border-width-heavy:  4px

--border-radius: 0px  /* ZAWSZE ZERO */
```

### 2.5 Shadows (Sharp, Layered)

```
--shadow-xs:     0 1px 2px rgba(0, 0, 0, 0.3)
--shadow-sm:     0 2px 4px rgba(0, 0, 0, 0.3)
--shadow-md:     0 4px 8px rgba(0, 0, 0, 0.35), 0 2px 4px rgba(0, 0, 0, 0.25)
--shadow-lg:     0 8px 16px rgba(0, 0, 0, 0.4), 0 4px 8px rgba(0, 0, 0, 0.3)
--shadow-xl:     0 12px 24px rgba(0, 0, 0, 0.45), 0 6px 12px rgba(0, 0, 0, 0.35)
--shadow-2xl:    0 16px 32px rgba(0, 0, 0, 0.5), 0 8px 16px rgba(0, 0, 0, 0.4)

/* Inset shadows dla depth */
--shadow-inset:  inset 0 1px 2px rgba(0, 0, 0, 0.4)
```

### 2.6 Transitions & Animations

```
--transition-fast:     150ms cubic-bezier(0.4, 0, 1, 1)
--transition-base:    200ms cubic-bezier(0.25, 0.46, 0.45, 0.94)
--transition-slow:     300ms cubic-bezier(0.25, 0.46, 0.45, 0.94)
--transition-slower:   400ms cubic-bezier(0.25, 0.46, 0.45, 0.94)

/* Easing functions - sharp, precise */
--ease-in:      cubic-bezier(0.4, 0, 1, 1)
--ease-out:     cubic-bezier(0, 0, 0.2, 1)
--ease-in-out:  cubic-bezier(0.25, 0.46, 0.45, 0.94)
```

### 2.7 Z-Index Scale

```
--z-index-base:      0
--z-index-dropdown:  1000
--z-index-sticky:    1100
--z-index-fixed:     1200
--z-index-modal:     1300
--z-index-popover:   1400
--z-index-tooltip:   1500
```

---

## 3. KOMPONENTY (Components)

### 3.1 Button

#### Variants
- **Primary**: Solid background, white text
- **Secondary**: Outlined, transparent background
- **Tertiary**: Text only, no background
- **Danger**: Red variant for destructive actions
- **Ghost**: Transparent with hover state

#### States
- Default
- Hover (background change + shadow)
- Active (pressed state)
- Focus (2px solid border)
- Disabled (opacity 0.5, no interaction)

#### Sizes
- **xs**: 24px height, 8px padding
- **sm**: 32px height, 12px padding
- **md**: 40px height, 16px padding (default)
- **lg**: 48px height, 20px padding
- **xl**: 56px height, 24px padding

#### Specs
```css
.button {
  border-radius: 0;
  border: 1px solid var(--color-border-primary);
  background: var(--color-bg-tertiary);
  color: var(--color-fg-primary);
  font-weight: var(--font-weight-medium);
  transition: all var(--transition-base);
  box-shadow: var(--shadow-xs);
}

.button:hover {
  background: var(--color-bg-hover);
  box-shadow: var(--shadow-sm);
  transform: translateY(-1px);
}

.button:active {
  transform: translateY(0);
  box-shadow: var(--shadow-xs);
}

.button:focus {
  outline: none;
  border-color: var(--color-border-focus);
  box-shadow: 0 0 0 2px var(--color-bg-focus);
}
```

### 3.2 Input / Text Field

#### States
- Default
- Hover (border color change)
- Focus (2px solid border + shadow)
- Error (red border)
- Disabled
- Read-only

#### Specs
```css
.input {
  border-radius: 0;
  border: 2px solid var(--color-border-primary);
  background: var(--color-bg-tertiary);
  color: var(--color-fg-primary);
  padding: var(--spacing-3) var(--spacing-4);
  font-size: var(--font-size-base);
  transition: all var(--transition-base);
}

.input:focus {
  border-color: var(--color-border-focus);
  box-shadow: 0 0 0 2px var(--color-bg-focus);
  outline: none;
}

.input:error {
  border-color: var(--color-border-error);
}
```

### 3.3 Card / Container

#### Variants
- **Default**: Standard elevation
- **Elevated**: Higher shadow
- **Outlined**: Border only, no shadow
- **Filled**: Solid background

#### Specs
```css
.card {
  border-radius: 0;
  border: 1px solid var(--color-border-primary);
  background: var(--color-bg-secondary);
  box-shadow: var(--shadow-md);
  padding: var(--spacing-6);
}
```

### 3.4 Modal / Dialog

#### Structure
- Backdrop (dark overlay)
- Container (centered, fixed)
- Header (title + close button)
- Body (scrollable content)
- Footer (actions)

#### Specs
```css
.modal-backdrop {
  background: rgba(0, 0, 0, 0.75);
  position: fixed;
  inset: 0;
  z-index: var(--z-index-modal);
}

.modal {
  border-radius: 0;
  border: 2px solid var(--color-border-primary);
  background: var(--color-bg-elevated);
  box-shadow: var(--shadow-2xl);
  max-width: 600px;
  max-height: 80vh;
  overflow: hidden;
}
```

### 3.5 Dropdown / Select

#### States
- Closed
- Open (with shadow)
- Hover (item highlight)
- Selected (background change)

#### Specs
```css
.dropdown {
  border-radius: 0;
  border: 2px solid var(--color-border-primary);
  background: var(--color-bg-elevated);
  box-shadow: var(--shadow-lg);
}

.dropdown-item {
  padding: var(--spacing-3) var(--spacing-4);
  border-bottom: 1px solid var(--color-border-primary);
}

.dropdown-item:hover {
  background: var(--color-bg-hover);
}

.dropdown-item:last-child {
  border-bottom: none;
}
```

### 3.6 Tooltip

#### Specs
```css
.tooltip {
  border-radius: 0;
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border-primary);
  color: var(--color-fg-primary);
  padding: var(--spacing-2) var(--spacing-3);
  font-size: var(--font-size-sm);
  box-shadow: var(--shadow-lg);
  z-index: var(--z-index-tooltip);
}
```

### 3.7 Sidebar Navigation

#### Structure
- Section headers (uppercase, small)
- Navigation items (icon + label)
- Active state (left border + background)
- Hover state (background change)

#### Specs
```css
.sidebar {
  border-radius: 0;
  background: var(--color-bg-secondary);
  border-right: 2px solid var(--color-border-primary);
  width: 240px;
}

.sidebar-item {
  padding: var(--spacing-3) var(--spacing-4);
  border-bottom: 1px solid var(--color-border-primary);
  transition: all var(--transition-base);
}

.sidebar-item:hover {
  background: var(--color-bg-hover);
}

.sidebar-item.active {
  background: var(--color-bg-active);
  border-left: 3px solid var(--color-fg-primary);
}
```

### 3.8 Table / List

#### Structure
- Header row (sticky, bold)
- Data rows (alternating backgrounds)
- Hover state
- Selected state

#### Specs
```css
.table {
  border-radius: 0;
  border: 1px solid var(--color-border-primary);
  width: 100%;
  border-collapse: collapse;
}

.table th {
  background: var(--color-bg-secondary);
  border-bottom: 2px solid var(--color-border-primary);
  padding: var(--spacing-3) var(--spacing-4);
  text-align: left;
  font-weight: var(--font-weight-semibold);
}

.table td {
  padding: var(--spacing-3) var(--spacing-4);
  border-bottom: 1px solid var(--color-border-primary);
}

.table tr:hover {
  background: var(--color-bg-hover);
}

.table tr.selected {
  background: var(--color-bg-active);
}
```

### 3.9 Toggle / Switch

#### States
- Off (gray background)
- On (white background)
- Disabled

#### Specs
```css
.toggle {
  border-radius: 0;
  width: 52px;
  height: 28px;
  border: 2px solid var(--color-border-primary);
  background: var(--color-bg-tertiary);
  position: relative;
  transition: all var(--transition-base);
}

.toggle-slider {
  border-radius: 0;
  width: 22px;
  height: 22px;
  background: var(--color-fg-secondary);
  position: absolute;
  top: 1px;
  left: 1px;
  transition: all var(--transition-base);
  box-shadow: var(--shadow-xs);
}

.toggle:checked {
  background: var(--color-fg-primary);
}

.toggle:checked .toggle-slider {
  transform: translateX(24px);
  background: var(--color-bg-primary);
}
```

### 3.10 Badge / Tag

#### Variants
- Default (gray)
- Primary (white)
- Success (green)
- Warning (yellow)
- Error (red)
- Info (blue)

#### Specs
```css
.badge {
  border-radius: 0;
  border: 1px solid var(--color-border-primary);
  background: var(--color-bg-tertiary);
  color: var(--color-fg-primary);
  padding: var(--spacing-1) var(--spacing-2);
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-medium);
  text-transform: uppercase;
  letter-spacing: var(--letter-spacing-wide);
}
```

---

## 4. LAYOUT SYSTEM

### 4.1 Grid System (12-column)

```
Container: max-width 1440px
Gutters: 24px
Columns: 12
Column width: calc((100% - 264px) / 12)
```

### 4.2 Breakpoints

```
--breakpoint-xs:   0px
--breakpoint-sm:   640px
--breakpoint-md:   768px
--breakpoint-lg:   1024px
--breakpoint-xl:   1280px
--breakpoint-2xl:  1536px
```

### 4.3 Container Widths

```
--container-xs:   100%
--container-sm:   640px
--container-md:   768px
--container-lg:   1024px
--container-xl:   1280px
--container-2xl:  1440px
```

---

## 5. ACCESSIBILITY (WCAG AAA)

### 5.1 Color Contrast
- Text on background: minimum 7:1
- Large text: minimum 4.5:1
- Interactive elements: minimum 3:1

### 5.2 Focus Indicators
- All interactive elements have visible focus
- Focus ring: 2px solid border
- Focus background: rgba(255, 255, 255, 0.12)

### 5.3 Keyboard Navigation
- Tab order: logical flow
- Skip links for main content
- All interactive elements keyboard accessible

### 5.4 Screen Reader Support
- Semantic HTML
- ARIA labels where needed
- Alt text for images
- Proper heading hierarchy

---

## 6. MICROINTERACTIONS

### 6.1 Hover Effects
- Background color change
- Shadow elevation
- Slight transform (translateY -1px)
- Border color change

### 6.2 Click/Press Effects
- Immediate feedback (150ms)
- Transform reset
- Shadow reduction

### 6.3 Loading States
- Skeleton screens (sharp edges)
- Progress indicators (linear, no rounded)
- Spinner (square rotation)

### 6.4 Transitions
- All state changes: 200ms
- Transform: cubic-bezier(0.25, 0.46, 0.45, 0.94)
- Color: linear

---

## 7. IMPLEMENTATION GUIDELINES

### 7.1 CSS Variables Usage
```css
/* ✅ DO */
.button {
  border-radius: 0;
  padding: var(--spacing-4);
  background: var(--color-bg-tertiary);
}

/* ❌ DON'T */
.button {
  border-radius: 4px;  /* ERROR: No border-radius allowed */
  padding: 16px;       /* Use spacing tokens */
  background: #1a1a1a;  /* Use color tokens */
}
```

### 7.2 Component Structure
```css
.component {
  /* Layout */
  display: flex;
  gap: var(--spacing-4);
  
  /* Spacing */
  padding: var(--spacing-4);
  margin: var(--spacing-2);
  
  /* Visual */
  border-radius: 0;  /* ALWAYS ZERO */
  border: var(--border-width-medium) solid var(--color-border-primary);
  background: var(--color-bg-secondary);
  box-shadow: var(--shadow-md);
  
  /* Typography */
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-regular);
  color: var(--color-fg-primary);
  
  /* Interaction */
  transition: all var(--transition-base);
}

.component:hover {
  background: var(--color-bg-hover);
  box-shadow: var(--shadow-lg);
}

.component:focus {
  outline: none;
  border-color: var(--color-border-focus);
  box-shadow: 0 0 0 2px var(--color-bg-focus);
}
```

### 7.3 Testing Checklist
- [ ] Zero border-radius on all elements
- [ ] WCAG AAA contrast compliance
- [ ] Keyboard navigation works
- [ ] Focus indicators visible
- [ ] Screen reader compatible
- [ ] Responsive on all breakpoints
- [ ] Consistent spacing (8px grid)
- [ ] All states defined (hover, focus, active, disabled)

---

## 8. DESIGN PRINCIPLES

1. **Function Over Form**: Every element serves a purpose
2. **Clarity**: Information hierarchy is obvious
3. **Consistency**: Same patterns throughout
4. **Precision**: Every pixel intentional
5. **Accessibility**: Usable by everyone
6. **Performance**: Fast, responsive interactions
7. **Scalability**: Works at any size

---

## 9. EXAMPLES

### 9.1 Button Group
```html
<div class="button-group">
  <button class="button button-primary">Primary</button>
  <button class="button button-secondary">Secondary</button>
  <button class="button button-tertiary">Tertiary</button>
</div>
```

### 9.2 Form Layout
```html
<form class="form">
  <div class="form-group">
    <label class="form-label">Email</label>
    <input type="email" class="input" placeholder="user@example.com">
  </div>
  <div class="form-group">
    <label class="form-label">Password</label>
    <input type="password" class="input">
  </div>
  <button type="submit" class="button button-primary">Submit</button>
</form>
```

### 9.3 Card Grid
```html
<div class="grid grid-cols-3 gap-6">
  <div class="card">
    <h3 class="card-title">Title</h3>
    <p class="card-content">Content</p>
  </div>
  <!-- ... -->
</div>
```

---

## 10. MIGRATION GUIDE

### Step 1: Update CSS Variables
Replace all hardcoded values with design tokens.

### Step 2: Remove All Border-Radius
```css
/* Find and replace */
border-radius: [any value] → border-radius: 0;
```

### Step 3: Update Component Styles
Apply new component specs to all UI elements.

### Step 4: Test Accessibility
Verify WCAG AAA compliance.

### Step 5: Visual QA
Check all states, breakpoints, and interactions.

---

**Version**: 1.0.0  
**Last Updated**: 2024  
**Maintained By**: Design System Team
